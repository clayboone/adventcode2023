# Separate users into "Ready" and "Not ready"
# ready_users = []
# not_ready_users = []
# for member in voice_channel.members:
#     if member in users_queue_sorted:
#         ready_users.append(member)
#     else:
#         not_ready_users.append(member)

# {member: member in users_queue_sorted for member in voice_channel.members}

# ready_users, not_ready_users = partition(
#     lambda member: member in users_queue_sorted, voice_channel.members
# )
# ready_users = {member for member in voice_channel.members if member in users_queue_sorted}
# not_ready_users = set(voice_channel.members) - ready_users

# not_ready_users = set(voice_channel.members) - (
#     ready_users := {
#         member for member in voice_channel.members if member in users_queue_sorted
#     }
# )

# def users(members, *, is_ready):
#     return {member for member in members if member in members_queue_sorted == is_ready}
# ready_users, not_ready_users = (users(voice_channel.members, is_ready=x) for x in (True, False))


# class Q:
#     x = 1

#     def set_instance_variable(self, x):
#         self.x = x

#     def set_class_variable(self, x):
#         self.__class__.x = x
#         # or Q.x = x


# th = Q()
# th.set_instance_variable(2)
# print(th.x, Q.x)

# th.set_class_variable(9_000)
# print(th.x, Q.x)

# a = "💃"
# b = "🎉"
# print(a + b)

# s1 = "asdf"
# try:
#     s1[1] = "b"
# except TypeError as exc:
#     print(f"Got exception: {exc}")

# print(s1)
# s2 = list(s1)
# s2[1] = "b"
# print("".join(s2))

# s = "😀😁😂🤣"
# print(s[3])




# try:
#     s = "a̸̧̧͚̥̻̬͍̝͙̳͋̓̇͐̂̋͐͛͊̀ṕ̷̖̫͙̒͑̈̾͒͛͗̕͠p̷̭͈̭̳̮̠̤̝͊̓̀ļ̷̛͔͕̬̣͚̹̩̠̳͑̀̑̽̊̐̽͋͊̐̓̾̚͜͝ę̷̻͔̤͖̙̺̼͕̞̫͍̺̳͙͙̯̓͐̅̆̇̈̓́͠s̷̫̦͖̞̖̩͔̦͖͓͐̉̐̏͗̿͑̅͌̊͝͠͠ ̴̛̹̿̑̿̆̍͐̉͊̑͒̾̈́̈̽̊͠ȧ̴̪̟̪̣̥̖̒͒͋͛̉̍̚̕͝͠ñ̴̛̥̽̎̇́̋̂̊̀̈́̀͂͛̈́̈́͒d̷̹͙͈̘̭̥̲̱̖̥̲͍͓̆̽̍̍̃̈́̑̄̅̀͐̕̕̕͝ ̶̢̦͗̀͒̔͒̌͌͛͘͠b̷̡̹̹͎̻̫͚͚̟͉̎̿̓͗̋̓̑̔͊͊̍̒̈͝a̸̡̧̛͚͇̱̭̞̥̺͇̭̳͚̣͂̂̐̐͝ͅņ̶̢̨̡̯̤̫͕̥͇̈͆̋̈̋̍͊̂̽̚͜͝à̷̡̛̦̙̖͔͔͕͖̮̼̹̟͈͍̥͑̚͠n̶̙̙͙͇̣̪̝̙̈̌̏͆̀̈́͛͊̋̑̓̍͝á̴̞̣̹̇̀̃̏̃̉̓͂̅̽̇̔͠s̵̨͔͇̣̩͎͓̱̥͂̉͋̍̂̈́͛̋̋̎̋͛̉̌̈͘̕"



#     print(s[4])
# except IndexError as exc:
#     print(f"Got exception: {exc}")

# import timeit

# def list_comprehension_test():
#     x = [1, 2] * 1000
#     q = {1}
#     y = [n for n in x if n in q]
#     z = [n for n in x if n not in q]

# def for_loop_test():
#     x = [1, 2] * 1000
#     q = {1}
#     y = []
#     z = []
#     for n in x:
#         if n in q:
#             y.append(n)
#         else:
#             z.append(n)

# def sets_reduction_test():
#     x = [1, 2] * 1000
#     q = {1}
#     y = set(x) & q
#     z = set(x) - q

# number = 1
# repeat = 100

# list_comp_time = min(timeit.repeat(list_comprehension_test, repeat=repeat, number=number))
# for_loop_time = min(timeit.repeat(for_loop_test, repeat=repeat, number=number))
# sets_reduction_time = min(timeit.repeat(sets_reduction_test, repeat=repeat, number=number))

# print(f"List comprehension: {list_comp_time * 1e6:,.2f}us")
# print(f"For loop: {for_loop_time * 1e6:,.2f}us")
# print(f"Sets reduction: {sets_reduction_time * 1e6:,.2f}us")

# print PID of process

# eval("os = __import__('os'); print(os.getpid())")
# eval("os = __import__('os'); print(os.getpid())")
# import os
# print(os.getpid())
# s = """\
# import os
# print(os.getpid())
# """
# eval(s)
# eval(s)
