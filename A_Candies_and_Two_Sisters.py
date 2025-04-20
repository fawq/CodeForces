def main() -> None:
    number_of_tests: int = int(input())

    for _ in range(number_of_tests):
        number_of_candies: int = int(input())
        if number_of_candies % 2 == 0:
            print(number_of_candies // 2 - 1)
        else:
            print(number_of_candies // 2)

if __name__ == "__main__":
    main()