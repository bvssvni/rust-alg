
LIB = src/lib.rs
TESTS = src/tests.rs

all: lib

testrun: lib $(TESTS)
	rustc --bin $(TESTS) -o bin/testrun -L bin/ && ./bin/testrun

test: lib $(TESTS)
	rustc --test $(TESTS) -o bin/tests -L bin/ && ./bin/tests

lib: $(LIB) bin
	rustc $(LIB) --out-dir bin/ -L bin/

bin:
	mkdir bin

doc: $(LIB)
	rustdoc $(LIB)

clean:
	rm bin/*
	cp lib/* -t bin/

