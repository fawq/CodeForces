def main() -> None:
    number: int = int(input())
    sign: int = (-1) ** number
    print((2 * sign * number + sign - 1) // 4)

if __name__ == "__main__":
    main()