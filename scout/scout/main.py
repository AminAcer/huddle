from nba_api.stats.endpoints import leaguestandings
from pymongo import MongoClient
import json
import os

def fetch_team_stats():
    league_standings = leaguestandings.LeagueStandings()

    standings_df = league_standings.get_data_frames()[0]

    team_stats = standings_df.to_dict(orient='records')
    return team_stats


def save_to_mongodb(data):
    mongo_uri = os.getenv("MONGO_URI", "mongodb://localhost:27017/nba_database")
    print(mongo_uri)
    client = MongoClient(mongo_uri)

    db = client.get_default_database()
    collection = db["team_stats"]

    collection.delete_many({})
    collection.insert_many(data)

    print("Team stats successfully saved to MongoDB")

def view_mongodb_data():
    mongo_uri = os.getenv("MONGO_URI", "mongodb://localhost:27017/nba_database")
    print(mongo_uri)
    client = MongoClient(mongo_uri)
    db = client.get_default_database()
    collection = db["team_stats"]

    for record in collection.find():
        print(record)


def main():
    stats = fetch_team_stats()
    save_to_mongodb(stats)
    view_mongodb_data()

if __name__ == "__main__":
    main()
