use log::info;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, PaginatorTrait, QueryFilter, QueryOrder};
use sea_orm::prelude::Uuid;
use thirtyfour::By;

use entity::prelude::target::ActionTargetKind;
use entity::test::ui::action::action;
use entity::test::ui::action::action::ActionKind;

use crate::error::{EngineError, EngineResult};
use crate::server::driver::UIHelper;

pub struct ActionController<'ccl>{
    db: &'ccl DatabaseConnection,
    driver: &'ccl UIHelper
}

impl<'ccl> ActionController<'ccl> {
    /// new - this will create new Action Controller for the application
    /// Creates a new instance of `ActionController` with the provided `db` and `drive` references.
    ///
    /// # Arguments
    ///
    /// * `db` - A reference to a `DatabaseConnection` object.
    /// * `drive` - A reference to a `UIHelper` object.
    ///
    /// # Example
    ///
    /// ```
    /// let db = DatabaseConnection::new();
    /// let driver = UIHelper::new();
    /// let controller = ActionController::new(&db, &driver);
    /// ```
    pub fn new(db: &'ccl DatabaseConnection, driver: &'ccl UIHelper) -> ActionController<'ccl> {
        Self{db, driver}
    }
    
    /// Asynchronous method that handles the logic for executing the "Open" action in a test case.
    ///
    /// # Arguments
    ///
    /// * `self` - A reference to the `ActionController` struct.
    /// * `action` - A reference to an `action::Model` object representing the action to be executed.
    ///
    /// # Example
    ///
    /// ```rust
    /// let action = action::Model::new();
    /// let controller = ActionController::new(&db, &driver);
    /// controller.command_open(&action).await;
    /// ```
    ///
    /// # Returns
    ///
    /// * `Result<(), EngineError>` - If the `data_value` field is `None`, it returns an `Err` with an `EngineError::Forbidden` variant. If the `data_value` field is not `None`, it opens the URL using the `drive` object and returns `Ok(())`.
    async fn command_open(&self, action: &action::Model) -> EngineResult<()> {
        let _action = match action.to_owned().data_value  {
            None => Err(EngineError::Forbidden),
            Some(url) => self.driver.open(url.as_str()).await
        };
        Ok(())
    }
    
    /// Asynchronously enters data into a target element on a web page using a WebDriver.
    ///
    /// # Arguments
    ///
    /// * `action` - An `action::Model` object that contains the necessary information for the action, including the data value to be entered, the target value of the element, and the target kind (CSS, ID, or XPath).
    ///
    /// # Example
    ///
    /// ```rust
    /// let action = action::Model {
    ///     data_value: Some("example data".to_string()),
    ///     target_value: Some("example target".to_string()),
    ///     target_kind: Some(ActionTargetKind::Css),
    /// };
    /// ui_helper.command_enter(&action).await;
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an `EngineError` if any of the required parameters (`action.data_value`, `action.target_value`, `action.target_kind`) are missing.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the action of entering data into the target element is successful.
    async fn command_enter(&self, action: &action::Model) -> EngineResult<()> {
        let data_value = action.data_value.clone().ok_or_else(|| EngineError::MissingParameter("action.data_value".to_string(), "".to_string()))?;
        let target_value = action.target_value.clone().ok_or_else(|| EngineError::MissingParameter("action.target_value".to_string(), "".to_string()))?;
        let target_kind = action.target_kind.clone().ok_or_else(|| EngineError::MissingParameter("action.target_kind".to_string(), "".to_string()))?;
        let by_kind = match target_kind {
            ActionTargetKind::Css => By::Css(target_value.as_str()),
            ActionTargetKind::Id => By::Id(target_value.as_str()),
            ActionTargetKind::Xpath => By::XPath(target_value.as_str())
        };
        self.driver.find(by_kind).await?.send_keys(data_value).await?;
        Ok(())
    }
    
    /// Performs a click action on a web element based on the provided target value and target kind.
    ///
    /// # Arguments
    ///
    /// * `action` - An instance of the `action::Model` struct that contains the target value and target kind.
    ///
    /// # Example
    ///
    /// ```rust
    /// let action = action::Model {
    ///     data_value: Some("button".to_string()),
    ///     target_kind: Some(ActionTargetKind::Css),
    /// };
    ///
    /// ui_helper.command_click(&action);
    /// ```
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the click action is performed successfully.
    async fn command_click(&self, action: &action::Model) -> EngineResult<()> {
        let target_value = action.data_value.clone()
            .ok_or_else(|| EngineError::MissingParameter("action.target_value".to_string(), "".to_string()))?;
        let target_kind = action.target_kind.clone()
            .ok_or_else(|| EngineError::MissingParameter("action.target_kind".to_string(), "".to_string()))?;
        let by_kind = match target_kind {
            ActionTargetKind::Css => By::Css(target_value.as_str()),
            ActionTargetKind::Id => By::Id(target_value.as_str()),
            ActionTargetKind::Xpath => By::XPath(target_value.as_str())
        };
        self.driver.find(by_kind).await?.click().await?;
        Ok(())
    }
    
    async fn command_verify_text(&self, action: &action::Model) -> EngineResult<()> {
        let data_value = action.data_value.clone()
            .ok_or_else(|| EngineError::MissingParameter("action.data_value".to_string(), "".to_string()))?;
        let target_value = action.target_value.clone()
            .ok_or_else(|| EngineError::MissingParameter("action.target_value".to_string(), "".to_string()))?;
        let target_kind = action.target_kind.clone()
            .ok_or_else(|| EngineError::MissingParameter("action.target_kind".to_string(), "".to_string()))?;
        let by_kind = match target_kind {
            ActionTargetKind::Css => By::Css(target_value.as_str()),
            ActionTargetKind::Id => By::Id(target_value.as_str()),
            ActionTargetKind::Xpath => By::XPath(target_value.as_str())
        };
        let text = self.driver.find(by_kind).await?.inner_html().await?;
        if text != data_value {
            info!("Verify text is failed {}", data_value);
            return Err(EngineError::MissingParameter("action.data_value".to_string(), data_value));
        }
        info!("Verify text is Success {}", data_value);
        Ok(())
    }
    
    pub async fn step_executor(&self, action: &action::Model) -> EngineResult<()> {
        let set_response = match action.kind { 
            ActionKind::Open => self.command_open(action).await?,
            ActionKind::Enter => self.command_enter(action).await?,
            ActionKind::Click => self.command_click(action).await?,
            ActionKind::DoubleClick => {},
            
            ActionKind::VerifyText =>  self.command_verify_text(action).await?,
            _ => {}
        };
        Ok(())
    }
    
    /// run_case - will execute the test case by the case ID
    pub async fn execute(&self, id: Uuid) -> EngineResult<()> {
        info!("Starting processing {action_id}", action_id=id);
        let mut action_page = action::Entity::find()
            .filter(action::Column::ActionGroupId.eq(id))
            .order_by_asc(action::Column::ExecutionOrder).paginate(self.db, 50);
        while let Some(actions) = action_page.fetch_and_next().await
            .map_err(EngineError::DatabaseError)? {
            for action in actions.into_iter() {
                info!("Executing step == [id] {:?}, [desc] {:?}", action.id, action.description);
                self.step_executor(&action).await?;
            }
        }
        Ok(())
    }
    
}

