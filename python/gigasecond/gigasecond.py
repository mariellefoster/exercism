from datetime import datetime, timedelta

def add(moment):
    delta = timedelta(seconds=1000000000)
    return moment + delta