check: python-check rust-check

python-check:
	python python/main.py < input > python.out
	diff python.out output

python-clean:
	rm -f python.out

rust-check:
	rustc rust/bin.rs -o rust/bin
	rust/bin < input > rust.out
	diff rust.out output

rust-clean:
	rm -f rust.out rust/bin

clean: python-clean rust-clean

.PHONY:
	python-check
	rust-check
