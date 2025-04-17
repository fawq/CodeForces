def get_number(n: int) -> int:
    even_numbers_indexes: list[int] = []
    odd_numbers_indexes: list[int] = []

    numbers: list[int] = list(map(int, input().split()))

    for index, number in enumerate(numbers):
        if number % 2 == 0:
            even_numbers_indexes.append(index)
        else:
            odd_numbers_indexes.append(index)       

    if len(even_numbers_indexes) == 1:
        return even_numbers_indexes[0] + 1
    else:
        return odd_numbers_indexes[0] + 1

def main() -> None:
    n: int = int(input())
    print(get_number(n))    

if __name__ == "__main__":
    main()