DENOMINATORS: list[int] = [100, 20, 10, 5, 1]

def main() -> None:
    money: int = int(input())
    total: int = 0

    for denominator in DENOMINATORS:
        total += money // denominator
        money %= denominator

    print(total)

if __name__ == "__main__":
    main()