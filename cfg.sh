cargo b --release
cp ./target/release/tcp /tmp/tcp
setcap cap_net_admin=eip /tmp/tcp
/tmp/tcp &
pid=$!
ip addr add 192.168.0.1/24 dev tun0
ip link set up dev tun0
trap "kill $pid" INT TERM
wait $pid
