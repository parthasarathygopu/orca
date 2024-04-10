use sea_orm::{ActiveModelTrait, DatabaseTransaction, EntityTrait, ModelTrait, Related, TryIntoModel};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::Uuid;
use tracing::info;

use cerium::client::Client;
use cerium::client::driver::web::WebDriver;
use cerium::client::storage::s3::S3Client;
use entity::prelude::{ActiveExecutionRequest, ActiveItemLog, CaseEntity, ExecutionKind, ExecutionRequest, ExecutionStatus, ExecutionType, ItemLog, ItemLogStatus, ItemLogType};
use entity::test::ui::suit::suite::{Column as SuiteColumn, Entity as SuiteEntity, Model as SuiteModel};
use entity::test::ui::suit::suite_block::{Column as BlockColumn, Entity as BlockEntity, Model as BlockModel, Model, SuiteBlockType};

use crate::controller::case::CaseController;
use crate::error::{EngineError, EngineResult};

pub struct SuiteController<'scl> {
    db: &'scl DatabaseTransaction,
    driver: WebDriver,
    client: Client,
    storage_cli: S3Client,
}

impl<'scl> SuiteController<'scl> {
    /// Constructs a new `ActionController` instance.
    ///
    /// # Arguments
    ///
    /// * `db` - A reference to a `DatabaseTransaction` instance.
    /// * `driver` - A `WebDriver` instance.
    /// * `client` - A `Client` instance.
    ///
    /// # Returns
    ///
    /// Returns a new `ActionController` instance.
    pub fn new(
        db: &'scl DatabaseTransaction,
        driver: WebDriver,
        client: Client,
    ) -> SuiteController<'scl> {
        let storage_cli = client.storage_cli.clone();
        Self {
            db,
            driver,
            client,
            storage_cli,
        }
    }

    /// execute - will execute the Suit based on the execution request
    pub async fn execute(&self, id: Uuid, er: &ExecutionRequest, log: Option<&ItemLog>) -> EngineResult<()> {
        let start = chrono::Utc::now();
        let log_id = log.map(|l| l.id);
        let mut action_log = ActiveItemLog::new(er.id, ItemLogType::TestSuite,
                                                id, log_id).save(self.db).await?;
        let suite = SuiteEntity::find_by_id(id).one(self.db).await?
            .ok_or(EngineError::MissingParameter("Suite".to_string(), id.into()))?;
        let log = action_log.clone().try_into_model()?;
        let result = self.execute_suite(&suite, er).await;

        action_log.finished_at = Set(chrono::Utc::now().into());
        action_log.execution_time = Set((chrono::Utc::now() - start).num_milliseconds() as i32);
        match result {
            Ok(_) => {
                action_log.status = Set(ItemLogStatus::Success);
                action_log.save(self.db).await?;
            }
            Err(e) => {
                action_log.status = Set(ItemLogStatus::Failed);
                action_log.save(self.db).await?;
                return Err(e);
            }
        }
        return Ok(());
    }


    pub async fn run(&self, id: Uuid, is_dry_run: bool) -> EngineResult<()> {
        let mut am_er = ActiveExecutionRequest::new(id, ExecutionType::TestSuite,
                                                    ExecutionKind::Trigger,
                                                    ExecutionStatus::Started, 0,
                                                    is_dry_run, Some(format!("[TS] Executing - {id}")), ).save(self.db).await?;
        let model_er = am_er.clone().try_into_model()?;
        info!("[{er}] Trigger Test Suite {suite_id}", er=model_er.id, suite_id = id);
        self.execute(id, &model_er, None).await?;
        am_er.finished_at = Set(chrono::Utc::now().into());
        am_er.status = Set(ExecutionStatus::Completed);
        am_er.save(self.db).await?;
        Ok(())
    }

    async fn switch_block<'a>(&self, ctrl: &'a CaseController<'a>, block: Model, er: &ExecutionRequest) -> EngineResult<()> {
        match block.type_field {
            SuiteBlockType::TestCase => {
                let case_id = block.reference.ok_or(EngineError::MissingParameter("Reference".to_string(), block.id.into()))?;
                info!("Switching to Test Case {block_id} - reference {case_id}",
                    block_id = block.id, case_id=case_id);
                ctrl.execute(case_id, er, None).await?;
                Ok(())
            }
        }
    }

    async fn execute_block(&self, block_id: Uuid, er: &ExecutionRequest) -> EngineResult<()> {
        info!("Executing Block {block_id}", block_id = block_id);
        let block = BlockEntity::find_by_id(block_id).one(self.db).await?
            .ok_or(EngineError::MissingParameter("SuiteBlock".to_string(), block_id.into()))?;

        Ok(())
    }

    async fn execute_suite(&self, suite: &SuiteModel, er: &ExecutionRequest) -> EngineResult<()> {
        info!("Executing Suite {suite_id}", suite_id = suite.id);
        let blocks = suite.find_related(BlockEntity).all(self.db).await?;
        let ctrl = CaseController::new(self.db, self.driver.clone(), self.client.clone());
        for block in blocks {
            self.switch_block(&ctrl, block, er).await?;
        }
        Ok(())
    }

    // /// run - will execute the test suite based on the execution request
    // /// # Arguments
    // /// * `id` - A `Uuid` instance.
    // /// * `is_dry_run` - A `bool` instance.
    // ///
    // /// # Returns
    // pub async fn run(&self, id: Uuid, is_dry_run: bool) -> EngineResult<()> {
    //     info!("Trigger Test Suite {suite_id}", suite_id = id);
    //     let start = chrono::Utc::now();
    //     let suite = SuiteEntity::find_by_id(id).one(self.db).await?
    //         .ok_or(EngineError::MissingParameter("Suite".to_string(), id.into()))?;
    //     Ok(())
    // }
}
