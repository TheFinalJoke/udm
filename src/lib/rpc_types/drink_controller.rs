// use crate::db::executor::GenQueries;
// use crate::error::UdmError;
use crate::rpc_types::drink_controller::drink_controller_service_server::DrinkControllerService;
use crate::rpc_types::drink_controller::drink_controller_service_server::DrinkControllerServiceServer;
use crate::rpc_types::drink_ctrl_types::CleanCycleRequest;
use crate::rpc_types::drink_ctrl_types::CleanCycleResponse;
// use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::vec;
// use crate::rpc_types::drink_ctrl_types::CleanType;
use crate::db::DbConnection;
use crate::db::DbMetaData;
use crate::db::DbType;
use crate::error::trace_log_error;
use crate::parsers::settings::UdmConfigurer;
use crate::rpc_types::drink_ctrl_types::DispenseDrinkRequest;
use crate::rpc_types::drink_ctrl_types::DispenseDrinkResponse;
use crate::rpc_types::drink_ctrl_types::GetPumpGpioInfoRequest;
use crate::rpc_types::drink_ctrl_types::GetPumpGpioInfoResponse;
use crate::rpc_types::drink_ctrl_types::PollDrinkStreamRequest;
use crate::rpc_types::drink_ctrl_types::PollDrinkStreamResponse;
use crate::rpc_types::drink_ctrl_types::PumpLogger;
use crate::rpc_types::drink_ctrl_types::ReqType;
use crate::rpc_types::fhs_types::FluidRegulator;
use crate::rpc_types::gpio_types::GpioDirection;
use crate::rpc_types::gpio_types::GpioMetadata;
use crate::rpc_types::gpio_types::GpioState;
use crate::rpc_types::server::GrpcServerFactory;
use crate::rpc_types::service_types::CollectFluidRegulatorsRequest;
use crate::rpc_types::service_types::FetchData;
use crate::rpc_types::service_types::GenericEmpty;
use crate::rpc_types::service_types::Operation;
use crate::rpc_types::service_types::ServiceResponse;
use crate::rpc_types::SqlUdmServerBuilder;
// use crate::system::gpio::PollSysDevice;
use crate::rpc_types::server::udm_service_client::UdmServiceClient;
use crate::system::gpio::GpioCollection;
use crate::system::gpio::GpioLineType;
use crate::UdmResult;
use futures::stream::StreamExt;
use signal_hook_tokio::SignalsInfo;
use std::net::SocketAddr;
use std::sync::Arc;
use tonic::async_trait;
use tonic::transport::channel::Channel;
use tonic::transport::Server;
use tonic::Request;
use tonic::Response;
use tonic::Status;
use tracing;
tonic::include_proto!("drink_ctrl_server");
pub struct DrinkControllerContext {
    pub connection: Box<dyn DbConnection>,
    pub addr: SocketAddr,
    pub metadata: DbMetaData,
    pub(crate) sql_udm_client: Option<UdmServiceClient<Channel>>,
    pub(crate) gpio_chip_path: String,
}
impl DrinkControllerContext {
    pub fn new(
        connection: Box<dyn DbConnection>,
        addr: SocketAddr,
        metadata: DbMetaData,
        sql_udm_client: Option<UdmServiceClient<Channel>>,
        gpio_chip_path: String,
    ) -> Self {
        Self {
            connection,
            addr,
            metadata,
            sql_udm_client,
            gpio_chip_path,
        }
    }
}
pub struct DrinkControllerServer {
    configuration: Arc<UdmConfigurer>,
    addr: SocketAddr,
}
impl DrinkControllerServer {
    pub fn new(config: Arc<UdmConfigurer>, addr: SocketAddr) -> Self {
        Self {
            configuration: config,
            addr,
        }
    }
}
#[async_trait]
impl GrpcServerFactory<DrinkControllerContext> for DrinkControllerServer {
    async fn build_context(&self) -> DrinkControllerContext {
        let db_type = Arc::new(DbType::load_db(Arc::clone(&self.configuration)));
        let mut connection = db_type.establish_connection().await;
        tracing::info!("Initializing database for the drink_controller");
        let _ = connection
            .gen_schmea_dc()
            .await
            .map_err(|e| format!("Failed to create database schema {e}"));
        tracing::info!("Attempting to Drink Controller Service on {}", self.addr);
        let db_metadata = DbMetaData::new(Arc::clone(&db_type));
        let sql_udm_options = SqlUdmServerBuilder::new(
            Ipv4Addr::new(127, 0, 0, 1).to_string(),
            self.configuration.udm.port,
        );
        DrinkControllerContext::new(
            connection,
            self.addr,
            db_metadata,
            sql_udm_options.connect().await.ok(),
            self.configuration.drink_controller.gpio_char_path.clone(),
        )
    }
    async fn start_server(&self) -> UdmResult<()> {
        let drink_contrl_server = self.build_context().await;
        let dc_service = DrinkControllerServiceServer::new(drink_contrl_server);
        tracing::info!("Running Drink Controller on {:?}", self.addr);
        let _ = Server::builder()
            .add_service(dc_service)
            .serve(self.addr)
            .await;
        Ok(())
    }
    async fn start_server_with_signal(&self, mut signal: SignalsInfo) -> UdmResult<()> {
        let drink_contrl_server = self.build_context().await;
        let dc_service = DrinkControllerServiceServer::new(drink_contrl_server);
        tracing::info!("Running Drink Controller Service on {:?}", self.addr);
        let _ = Server::builder()
            .add_service(dc_service)
            .serve_with_shutdown(self.addr, async {
                let _ = signal.next().await;
                tracing::info!("Got a termination signal");
            })
            .await;
        Ok(())
    }
}

// Need to figure out locking
#[async_trait]
impl DrinkControllerService for DrinkControllerContext {
    async fn dispense_drink(
        &self,
        request: Request<DispenseDrinkRequest>,
    ) -> Result<Response<DispenseDrinkResponse>, Status> {
        tracing::debug!("Got request {request:?}");
        // first check if pump is running
        // collect all the pump info
        // set the gpio pin
        // return data
        todo!()
    }
    async fn clean_cycle(
        &self,
        _request: Request<CleanCycleRequest>,
    ) -> Result<Response<CleanCycleResponse>, Status> {
        todo!()
    }
    async fn get_pump_gpio_info(
        &self,
        request: Request<GetPumpGpioInfoRequest>,
    ) -> Result<Response<GetPumpGpioInfoResponse>, Status> {
        tracing::info!("Got request {request:?}");
        let uuid = PumpLogger::create_uuid();
        let fr = request
            .get_ref()
            .fr
            .clone()
            .ok_or(trace_log_error(Status::invalid_argument(
                "Missing Fluid regulator".to_string(),
            )))?;
        let uuid = PumpLogger::new(Some(uuid), ReqType::GetPumpInfo, fr.fr_id)
            .publish(&*self.connection)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;
        let fetch_query = {
            if let Some(pump_num) = fr.pump_num {
                CollectFluidRegulatorsRequest::builder()
                    .expressions(vec![FetchData::builder()
                        .column("pump_num".to_string())
                        .operation(Operation::Equal.into())
                        .values(pump_num.to_string())
                        .build()])
                    .build()
            } else if let Some(gpio_pin) = fr.gpio_pin {
                CollectFluidRegulatorsRequest::builder()
                    .expressions(vec![FetchData::builder()
                        .column("gpio_pin".to_string())
                        .operation(Operation::Equal.into())
                        .values(gpio_pin.to_string())
                        .build()])
                    .build()
            } else if let Some(gpio_name) = fr.gpio_name {
                CollectFluidRegulatorsRequest::builder()
                    .expressions(vec![FetchData::builder()
                        .column("gpio_name".to_string())
                        .operation(Operation::Equal.into())
                        .values(gpio_name.to_string())
                        .build()])
                    .build()
            } else {
                Err(Status::aborted(
                    "Error trying to collect the FR from the database",
                ))?
            }
        };
        let mut sql_client = self.sql_udm_client.clone().unwrap();
        let result = sql_client.collect_fluid_regulators(fetch_query).await?;
        tracing::debug!("Fetched Fluid Regulators: {:?}", result.get_ref());
        if result.get_ref().fluids.is_empty() || result.get_ref().fluids.len() > 2 {
            return Err(Status::aborted(format!(
                "Returned no data or too much data {result:?}"
            )));
        }
        let line = GpioLineType::try_from(result.get_ref().fluids[0].clone()).map_err(|e| {
            Status::internal(format!("Failed to get GPIO line from fluid regulator: {e}"))
        })?;

        let mut gpio_collection = GpioCollection::new(line, self.gpio_chip_path.clone());
        gpio_collection.poll().map_err(|e| {
            Status::internal(format!(
                "Failed to poll GPIO line from fluid regulator: {e}"
            ))
        })?;
        Ok(GetPumpGpioInfoResponse::builder()
            .metadata(
                gpio_collection
                    .metadata
                    .ok_or(trace_log_error(Status::internal(
                        "Failed to get GPIO metadata".to_string(),
                    )))?,
            )
            .id(uuid.to_string())
            .build()
            .to_response())
    }
    async fn stop_emergency(
        &self,
        _request: Request<FluidRegulator>,
    ) -> Result<Response<GenericEmpty>, Status> {
        todo!()
    }
    async fn poll_drink_stream(
        &self,
        _request: Request<PollDrinkStreamRequest>,
    ) -> Result<Response<PollDrinkStreamResponse>, Status> {
        todo!()
    }
}
