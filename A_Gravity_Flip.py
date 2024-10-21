def main() -> None:
    _ = input()
    numbers: list[int] = sorted(map(int, input().split()))

    print(" ".join(map(str, numbers)))

if __name__ == "__main__":
    main()