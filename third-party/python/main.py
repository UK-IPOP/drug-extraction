from __future__ import annotations

import utils


def main():
    algorithm = utils.get_user_input()
    input_file = utils.load_data()
    utils.runner(search_metric=algorithm, input_file=input_file)


if __name__ == "__main__":
    print("Starting program...")
    main()
