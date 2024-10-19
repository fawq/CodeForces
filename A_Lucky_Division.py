LUCKY_NUMBERS: list[int] = [4, 7, 44, 47, 74, 77, 444, 447, 474, 477, 744, 747, 774, 777]

def main() -> None:
    number: int = int(input())

    for lucky_number in LUCKY_NUMBERS:
        if number % lucky_number == 0:
            print("YES")
            break
    else:
        print("NO")

if __name__ == "__main__":
    main()