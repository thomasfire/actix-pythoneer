from sys import argv


def get_sign(x: int):
    if x > 0:
        return 1
    elif x < 0:
        return -1
    return 0

def main(expression: str):
    try:
        abs_expr = expression.replace("(", "abs(")
        result = eval(abs_expr)
        sign = get_sign(eval(expression))
        return "{}\n{}".format("1", result * sign)
    except Exception as e:
        return "0"


if __name__ == '__main__':
    if len(argv) < 2:
        print("0")
        exit(0)
    print(main(argv[1]))
