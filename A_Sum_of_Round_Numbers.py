def print_summands(number) -> None:
    summands: list[int] = []
    scalar: int = 10

    while number > 0:
        digit_with_zeros = number % scalar
        number -= digit_with_zeros
        
        if digit_with_zeros > 0:
            summands.append(digit_with_zeros)
        
        scalar *= 10

    print(len(summands))
    print(" ".join(map(str, summands)))

def main() -> None:
    number_of_questions: int = int(input())

    for _ in range(number_of_questions):
        number = int(input())
        print_summands(number)

if __name__ == "__main__":
    main()