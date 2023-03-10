use clap::Parser;
use clap::ArgGroup;
use dotenv::dotenv;
use std::path::Path;
use home_dir::*;

///netcatの動作モード。
///TCP・UDPサーバー・クライアントとポートスキャンなどがある。
enum NcMode {
    ///TCPクライアント
    TcpClient,
    ///UDPクライアント
    UdpClient,
    ///TCPサーバー
    TcpServer,
    ///UDPサーバー
    UdpServer,
    ///ポートスキャン
    PortScan,
}

#[derive(Debug, Parser)]
#[command(group(ArgGroup::new("protocol").required(false).args(["tcp", "udp"])))]
pub struct Cli {
    #[arg(name="tcp", short='t', long="tcp", help="TCPクライアント・サーバー", env="RSNC_TCP")]
    tcp: bool,
    #[arg(name="udp", short='u', long="udp", env="RSNC_UDP", help="UDPクライアント・サーバー")]
    udp: bool,
    #[arg(name="listen", short='l', long="listen", help="サーバーモード", env="RSNC_LISTEN")]
    listen: bool,
    #[arg(name="keep-accept", short='k', long="keep-accept", help="クライアントが閉じても終了しない。", env="RSNC_KEEP_ACCEPT")]
    keep_accept: bool,
    #[arg(value_name="port", index=1, help="ポート番号", env="RSNC_PORT")]
    port: Option<u16>,
    #[arg(value_name="ipaddr", index=2, help="IPアドレス", env="RSNC_IPADDR")]
    ipaddr: Option<String>

}

/// ユーザーの$HOMEの環境変数ファイル
const HOME_DOTENV_FNAME: &str = ".rsncenv"; 
/// dotenvの設定
/// 環境変数ファイルは$HOMEの`HOME_DOTENV_FNAME`とカレントディレクトリの.envファイル。
/// #Example
/// set_dotenv();
///
fn set_dotenv() {
    let path = Path::new(HOME_DOTENV_FNAME).expand_home().unwrap();
    if path.exists() {
        dotenv::from_path(path);
    } else {
        dotenv().ok();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    ///コマンドライン解析テスト
    #[test]
    fn test_cli_default() {
        let args = Cli::try_parse_from([""]);
        let args = args.unwrap();
        assert_eq!(false, args.tcp);
        assert_eq!(false, args.udp);
        assert_eq!(None, args.port);
        assert_eq!(None, args.ipaddr);
    }
    #[test]
    fn test_cli() {
        let args = Cli::try_parse_from(["", "-u", "-l", "54321", "127.0.0.1"]);
        let args = args.unwrap();
        assert_eq!(false, args.tcp);
        assert_eq!(true, args.udp);
        assert_eq!(54321, args.port.unwrap());
        assert_eq!("127.0.0.1".to_string(), args.ipaddr.unwrap());
        
    }

    #[test]
    fn test_dotenv() {
        set_dotenv();
        let args = Cli::try_parse_from([""]);
        let args = args.unwrap();
        assert_eq!(true, args.tcp);
        assert_eq!(false, args.udp);
        assert_eq!(true, args.listen);
        assert_eq!(54321, args.port.unwrap());
        assert_eq!("127.0.0.1".to_string(), args.ipaddr.unwrap());
        
    }
}
