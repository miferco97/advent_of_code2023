import os,sys
import json
import matplotlib.pyplot as plt
import datetime

def print_day_vector(day_vector:list):
    for day, stars in enumerate(day_vector):
        first, second = stars
        # convert unix timestamp to datetime
        first = datetime.datetime.fromtimestamp(first) if first > 0 else "N/A"
        second = datetime.datetime.fromtimestamp(second) if second > 0 else "N/A"
        if first == "N/A" and second == "N/A":
            continue
        print(f"Day {day+1}: {first} :: {second}")

def generate_day_vector(member:dict) -> list:
    day_vector = [ [0,0] for i in range(25)]  # 25 days
    for day, part in member_data.items():
        index = int(day[0])-1
        for part, ts in part.items():
            part = int(part[-1])-1
            day_vector[index][part] = ts
    return day_vector.copy()

# tuple_name = ("name", "day", "part", "ts")

if __name__ == '__main__':
    dict_members = {}
    args = sys.argv
    if len(args) != 2:
        print("Usage: python visualizer.py <path_to_log_file>")
        exit(1)
    path = args[1]
    # parse json file
    with open(path, 'r') as f:
        data = json.load(f)
    for id, member in data["members"].items():
        dict_members[member["name"]] = {}
        for day, day_data in member["completion_day_level"].items():
            if day not in dict_members[member["name"]]:
                dict_members[member["name"]][day] = {}
            for day_part, day_part_data in day_data.items():
                dict_members[member["name"]][day][day_part] = int(day_part_data["get_star_ts"])

    tuples_list = []
    for member, member_data in dict_members.items():
        day_vector = generate_day_vector(member_data)
        for day,part in enumerate(day_vector):
            for part,ts in enumerate(part):
                    tuple = (member, day, part, ts)
                    tuples_list.append(tuple)
        # print_day_vector(day_vector)
        # print(tuples_list)

    people = list(set(map(lambda x: x[0], tuples_list)))
    
    for day in range(25):
        day_tuples = list(filter(lambda x: x[1] == day, tuples_list))
        star1 = list(filter(lambda x: x[2] == 0, day_tuples))
        star2 = list(filter(lambda x: x[2] == 1, day_tuples))

        print(f"Day {day+1}:")
        star1_rank = sorted(star1, key=lambda x: x[3] if x[3] > 0 else 9999999999999)
        star2_rank = sorted(star2, key=lambda x: x[3] if x[3] > 0 else 9999999999999)

        print(f"Star 1: {star1_rank[0:3]}")
        print(f"Star 2: {star2_rank[0:3]}")
        
