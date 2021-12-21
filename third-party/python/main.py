from __future__ import annotations

import utils


def main():
    algorithm = utils.get_user_input()
    dataset = utils.load_data()
    utils.runner(search_metric=algorithm, data=dataset)


if __name__ == "__main__":
    print("Starting program...")
    main()
