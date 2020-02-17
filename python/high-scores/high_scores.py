def latest(scores):
    return scores[len(scores)-1]


def personal_best(scores):
    return max(scores)


def personal_top_three(scores):
    if len(scores) > 3:
        return sorted(scores, reverse=True)[:3]
    return sorted(scores, reverse=True)
