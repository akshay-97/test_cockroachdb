insert_map = {}
find_map = {}
update_map = {}

def check_and_add(m, table, latency):
    if table in m:
        m[table].append(latency)
    else:
        m[table] = [latency]

file_path = 'output_2' 
with open(file_path, 'r') as file:
    for line in file:
        words = line.split()
        if words[1] == "INSERT":
            check_and_add(insert_map, words[0], int(words[2]))
        elif words[1] == "FIND":
            check_and_add(find_map, words[0], int(words[2]))
        elif words[1] == "UPDATE":
            check_and_add(update_map, words[0], int(words[2]))


def get_tp_index(len):
    return ((len*95)//100, (len*50)//100)

print("INSERT STATS")
for i in insert_map:
    insert_map[i].sort()
    avg = sum(insert_map[i])/len(insert_map[i])

    (p95, p50) = get_tp_index(len(insert_map[i]))

    print(f"table {i} insights -------")
    print(f"Avg latency : {avg}")
    print(f"tp95 stats: {insert_map[i][p95]}")
    print(f"tp50 stats: {insert_map[i][p50]}")

print("\n")
print("FIND STATS")
for i in find_map:
    find_map[i].sort()
    avg = sum(find_map[i])/len(find_map[i])

    (p95, p50) = get_tp_index(len(find_map[i]))

    print(f"table {i} insights -------")
    print(f"Avg latency : {avg}")
    print(f"tp95 stats: {find_map[i][p95]}")
    print(f"tp50 stats: {find_map[i][p50]}")


print("\n")
print("UPDATE STATS")
for i in update_map:
    update_map[i].sort()
    avg = sum(update_map[i])/len(update_map[i])

    (p95, p50) = get_tp_index(len(update_map[i]))

    print(f"table {i} insights -------")
    print(f"Avg latency : {avg}")
    print(f"tp95 stats: {update_map[i][p95]}")
    print(f"tp50 stats: {update_map[i][p50]}")