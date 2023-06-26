/* Declares the app's configuration based on it's environment. */

pub enum AppEnvironment {
    Local,
    Prod,
}

pub struct AppConfig {
    api_url: String,
    env: AppEnvironment,
}
