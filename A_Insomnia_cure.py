def main() -> None:
    k: int = int(input())
    l: int = int(input())
    m: int = int(input())
    n: int = int(input())
    d: int = int(input())
    damaged_dragons: int = 0

    for number in range(1, d + 1):
        if number % k == 0 or number % l == 0 or number % m == 0 or number % n == 0:
            damaged_dragons += 1

    print(damaged_dragons)

if __name__ == "__main__":
    main()