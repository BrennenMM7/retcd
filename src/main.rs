mod etcd;
fn main() {
    etcd::etcd::start_etcd_or_proxy_v2();
}