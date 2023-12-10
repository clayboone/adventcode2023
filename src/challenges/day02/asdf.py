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

not_ready_users = set(voice_channel.members) - (
    ready_users := {
        member for member in voice_channel.members if member in users_queue_sorted
    }
)

def users(members, *, is_ready):
    return {member for member in members if member in members_queue_sorted == is_ready}
ready_users, not_ready_users = (users(voice_channel.members, is_ready=x) for x in (True, False))
