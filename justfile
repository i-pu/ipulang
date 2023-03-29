compile FILE:
    cargo run --bin ipulang-compiler -- test_codes/{{FILE}}.ipu --output test_codes/{{FILE}}.ll
    llc-12 ./test_codes/{{FILE}}.ll
    gcc ./test_codes/{{FILE}}.s -o ./test_codes/{{FILE}}.out

vm FILE:
    cargo run --bin ipulang-compiler -- test_codes/{{FILE}}.ipu --output out.bytecode

run FILE:
    just compile {{FILE}}
    ./test_codes/{{FILE}}.out