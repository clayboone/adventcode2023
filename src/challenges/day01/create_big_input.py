import argparse
import os


def none_existing_file(filename):
    if os.path.exists(filename):
        raise argparse.ArgumentTypeError(f"File {filename} already exists")
    return filename


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "filename", help="Name of the file to create", type=none_existing_file
    )
    parser.add_argument("size", help="Size of the input file, in megabytes", type=int)
    args = parser.parse_args()

    with open(args.filename, "w") as f:
        for _ in range(args.size * 1024):
            f.write("oneEEEEEEEEEzero" * 64 + "\n")  # 1KB per line


if __name__ == "__main__":
    main()
