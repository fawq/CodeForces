def main() -> None:
    number_of_friends: int = int(input())
    presents_origin: list[int] = [None] * number_of_friends
    presents_destination: list[int] = map(int, input().split())

    for index, present in enumerate(presents_destination):
        presents_origin[present - 1] = index + 1
    
    print(" ".join(map(str, presents_origin)))

if __name__ == "__main__":
    main()