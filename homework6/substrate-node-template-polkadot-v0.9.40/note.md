# notes

* code reference

~~~
https://github.com/kaichaosun/play-substrate/blob/master/pallets/poe/src/tests.rs
~~~

* benchmark compile
~~~
cargo build --release --features runtime-benchmarks
~~~

* benchmark run

~~~
./target/release/node-template benchmark pallet \
--chain dev \
--execution wasm \
--wasm-execution compiled \
--pallet pallet_poe --extrinsic "*" \
--steps 20 --repeat 10 \
--output ./pallets/poe/src/weights.rs \
--template .maintain/frame-weight-template.hbs
~~~


~~~
 ./target/release/node-template benchmark pallet \
> --chain dev \
> --execution wasm \
> --wasm-execution compiled \
> --pallet pallet_poe --extrinsic "*" \
> --steps 20 --repeat 10 \
> --output ./pallets/poe/src/weights.rs \
> --template .maintain/frame-weight-template.hbs
~~~