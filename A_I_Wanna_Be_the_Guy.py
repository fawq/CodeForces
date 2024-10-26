PASS: str = "I become the guy."
FAIL: str = "Oh, my keyboard!"

def main() -> None:
    levels: int = int(input())
    levels_passed: set[int] = set()

    x_levels = list(map(int, input().split()))
    y_levels = list(map(int, input().split()))

    for index in range(1, len(x_levels)):
        levels_passed.add(x_levels[index])

    for index in range(1, len(y_levels)):
        levels_passed.add(y_levels[index])

    print(PASS if len(levels_passed) == levels else FAIL)

if __name__ == "__main__":
    main()