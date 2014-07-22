check: python-check rust-check

python-run:
	python python/main.py < input | tee python.out

python-check: python-run
	diff python.out output

python-clean:
	rm -f python.out

rust-run:
	rustc rust/bin.rs -o rust/bin
	rust/bin < input | tee rust.out

rust-check: rust-run
	diff rust.out output

rust-clean:
	rm -f rust.out rust/bin

clean: python-clean rust-clean

.PHONY:
	python-run
	python-check
	rust-run
	rust-check
