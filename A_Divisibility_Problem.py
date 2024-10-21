def main() -> None:
    number_of_test_cases: int = int(input())

    for _ in range(number_of_test_cases):
        a, b = map(int, input().split())
        print("0" if a % b == 0 else f"{b - a % b}")

if __name__ == "__main__":
    main()