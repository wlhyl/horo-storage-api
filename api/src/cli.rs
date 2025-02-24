use std::net::Ipv4Addr;
use std::num::NonZeroUsize;
use clap::Parser;

/// 服务器配置参数
#[derive(Parser, Debug)]
#[clap(
    author = "Your Name <your.email@example.com>",
    version,
    about = "Horoscope Storage REST API Service",
    long_about = None
)]
pub struct ServerConfig {
    /// 服务器监听地址
    /// 示例: 127.0.0.1 表示本地访问, 0.0.0.0 表示所有地址
    #[clap(short, long, value_parser, default_value = "0.0.0.0")]
    pub ip: Ipv4Addr,

    /// 服务器端口号 (1024-65535)
    #[clap(
        short, 
        long, 
        value_parser = clap::value_parser!(u16).range(1024..65536),
        default_value_t = 8080
    )]
    pub port: u16,

    /// 工作线程数量 (最小值: 1)
    /// 建议设置为 CPU 核心数
    #[clap(
        short = 'w',
        long = "workers",
        value_parser,
        default_value_t = NonZeroUsize::new(1).unwrap()
    )]
    pub workers: NonZeroUsize,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self::parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        ServerConfig::command().debug_assert()
    }

    #[test]
    fn test_default_values() {
        let config = ServerConfig::default();
        assert_eq!(config.ip.to_string(), "0.0.0.0");
        assert_eq!(config.port, 8080);
        assert!(config.workers.get() > 0);
    }
}
