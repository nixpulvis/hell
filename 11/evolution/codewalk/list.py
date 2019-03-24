# Create a list `grades`.
grades = ["A", "B", "C", "D", "E", "F"]

# Print the list
print grades

# Print the list mapped with "+" added to each grade.
mapped = map(lambda g: "{}+".format(g), grades)
print mapped

# Print the list mapped and reversed.
mapped.reverse()
print mapped
