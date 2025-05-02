def check_if_prime(number: int) -> bool:
    if number == 1:
        return False

    if number == 2:
        return True

    if number % 2 == 0:
        return False

    i = 3
    while i * i <= number:
        if number % i == 0:
            return False
        i += 2

    return True

def check_if_square(number: int) -> bool:
    return int(number ** 0.5) ** 2 == number

def print_if_t_prime(number: int) -> None:
    if check_if_square(number) and check_if_prime(int(number ** 0.5)):
        print("YES")
    else:
        print("NO")

def main() -> None:
    _ = input()
    numbers: list[int] = list(map(int, input().split()))

    for number in numbers: 
        print_if_t_prime(number)

if __name__ == "__main__":
    main()