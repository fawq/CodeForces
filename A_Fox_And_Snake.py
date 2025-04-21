def main() -> None:
    n, m = map(int, input().split())
    
    for index in range(n):
        if index % 2 == 0:
            print("#" * m)
        elif index % 4 == 1:
            print("." * (m - 1) + "#")
        else:
            print("#" + "." * (m - 1))

if __name__ == "__main__":
    main()