from sys import argv


def main(expression: str):
    try:
        return "{}\n{}".format("1", eval(expression))
    except Exception as e:
        return "0"


if __name__ == '__main__':
    if len(argv) < 2:
        print("0")
        exit(0)
    print(main(argv[1]))
