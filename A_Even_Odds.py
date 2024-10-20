def main() -> None:
    n, k = map(int, input().split())

    index_of_change: int = n // 2 if n % 2 == 0 else (n + 1) // 2
    if k <= index_of_change:
        print(2 * k - 1)
    else:
        print(2 * (k - index_of_change))

if __name__ == "__main__":
    main()