def main() -> None:
    a: int = int(input())
    b: int = int(input())
    c: int = int(input())

    result_1: int = a + b + c
    result_2: int = a + b * c
    result_3: int = a * b + c
    result_4: int = a * b * c
    result_5: int = (a + b) * c
    result_6: int = a * (b + c)

    print(max(result_1, result_2, result_3, result_4, result_5, result_6))

if __name__ == "__main__":
    main()