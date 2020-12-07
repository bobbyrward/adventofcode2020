def get_input():
    with open("../../inputs/day07.txt") as fd:
        return (x.strip() for x in fd.readlines())


def parse_line(line):
    (outer, inner) = line[:-1].split(" contain ")

    if outer.endswith("bags"):
        outer = outer[:-5]
    else:
        outer = outer[:-4]

    if inner == "no other bags":
        return outer, {}

    contents = {}

    for bag in inner.split(","):
        n, bag = bag.strip().split(" ", 1)

        if bag.endswith("bags"):
            bag = bag[:-5]
        else:
            bag = bag[:-4]

        contents[bag] = int(n)

    return outer, contents


# striped beige bags contain no other bags.
# clear lavender bags contain 2 pale violet bags, 5 clear yellow bags, 5 striped salmon bags.

SHINY_GOLD = "shiny gold"


def find_gold_bag_containers(bags, bag, contents, state):
    # import pprint

    # pprint.pprint(bags[bag])

    if SHINY_GOLD in bags[bag]:
        state.add(bag)
        return
    else:
        for child in contents:
            child_contents = bags[child]
            find_gold_bag_containers(bags, child, child_contents, state)

            if child in state:
                state.add(bag)
                return


def main():
    lines = [parse_line(line) for line in get_input()]
    bags = {bag: contents for bag, contents in lines}

    # import pprint
    # pprint.pprint(bags)

    state = set()
    for bag, contents in bags.items():
        find_gold_bag_containers(bags, bag, contents, state)

    print(len(state))


if __name__ == "__main__":
    main()
