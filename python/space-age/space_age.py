class SpaceAge:
    def __init__(self, seconds):
        # - Earth: orbital period 365.25 Earth days, or 31557600 seconds
        self.seconds = seconds
        self.earth = 31557600

    def on_earth(self):
        return round(self.seconds / self.earth, 2)

    def on_mercury(self):
        return round(self.seconds / (self.earth * 0.2408467), 2)

    def on_venus(self):
        return round(self.seconds / (self.earth * 0.61519726), 2)

    def on_mars(self):
        return round(self.seconds / (self.earth * 1.8808158), 2)

    def on_jupiter(self):
        return round(self.seconds / (self.earth * 11.86261), 2)

    def on_saturn(self):
        return round(self.seconds / (self.earth * 29.447498), 2)

    def on_uranus(self):
        return round(self.seconds / (self.earth * 84.016846), 2)

    def on_neptune(self):
        return round(self.seconds / (self.earth * 164.79132 ), 2)

       # - Jupiter: orbital period 11.862615 Earth years
       # - Saturn: orbital period 29.447498 Earth years
       # - Uranus: orbital period 84.016846 Earth years
       # - Neptune: orbital period 164.79132 Earth years


# load all planets


# do a seconds to years calculation