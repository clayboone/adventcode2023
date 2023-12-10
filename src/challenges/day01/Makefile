.PHONY all: create_test_data

create_test_data:
	@echo "creating test data..."
	python create_big_input.py big_input_10mb.txt 10
	python create_big_input.py big_input_100mb.txt 100
	python create_big_input.py big_input_1000mb.txt 1000

	@# I never liked my SSDs anyway...
	python create_big_input.py big_input_10000mb.txt 10000
