def main() -> None:
    numbers: list[int] = list(map(int, input().split()))
    numbers.sort()
    print(numbers[2] - numbers[0])

if __name__ == "__main__":
    main()