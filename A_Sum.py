def print_if_sum_of_others(a: int, b: int, c: int) -> None:
    if a + b == c or a + c == b or b + c == a:
        print("YES")
    else:
        print("NO")

def main() -> None:
    number_of_test_cases: int = int(input())

    for _ in range(number_of_test_cases):
        a, b, c = map(int, input().split())
        print_if_sum_of_others(a, b, c)

if __name__ == "__main__":
    main()