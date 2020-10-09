# Crate Meter

This simulates the electric power meter of a household. It returns
the instantaneous power the consumer draws.

A number `2 * n` normal distributed values are (pseudo) randomly generated and
split in two sets `xs = {x}, ys = {abs(y)}`. The _xs_ are normalized such that
`max(abs( x ))` equals a little less than half the active time. While the _ys_
are normalized to to the interval of base and peak power.

Values _xs_ and _ys_ are arbitrarily paired to `ps = {(x,y)}` and sorted. 
Points at base power are added for start and end of active time.

The power over time curve is an interpolation between points _ps_.

Time is propagated mockingly by looping with sampling interval sized steps over
the mock interval range, starting at 0 seconds (midnight). If the interval is
longer than a day the whole simulation function is called recursively with the
remainder.

Sends timestamp and present power via rabbitMQ to PV.

## Parameters:
- peak power [W]
- base power [W]
- sampling interval _s_ [s]
- activity start [h]
- activity end [h]
- number of points _n_
- mock date [d], days since 2020-01-01
- mock interval [s] (default 86400)

## improvements
- use proper chrono::DateTime struct
- mock system time functions


# Crate PV
This simulates a photovoltaics system including including interface to its
consumer and the power grid.



Its parameters are:
- the nominal power of the pv array
- azimuth [deg] and tilt [deg] of the panel
  - azimuth 180 is a south facing panel
  - at a tilt of 0 a panel is horizontal
- efficiency (we calculate panel area with it)


# Crate Solarize
Solarize simulates the instantaneous insolation. It takes date [day number],
local solar time [s], and decimal latitude. It returns instantaneous
insolation [W/m^2] and zenith angle _z_ [rad] and azimuth angle _a_ [rad].

Zenith angle is calculated by: [1]

   cos z =sin l sin +cosϕ cos d cos h

where:
- latitude _l_ in rad
- declination _d_ (date)
- hour angle _h_ (0 at noon, 15 degree per hour, `15 * 60^(-3) * pi` [1/s])

The suns' azimuth equals the hour angle for afternoons in the the northern
hemisphere.  Trivial corrections yield all other cases.

##Approximations:
- the sun is a considered a point source: at `cos z <= 0` there is no insolation
- no atmospheric effects but attenuation
- air-mass is approximated by `min(sec z, 40)`
  - air mass is relative attenuation compared to horizon
  - plane parallel approximation
  - cut off at 40 to address earths curvature
  - solar panel peak power is usually give for 1.5 air mass
- no weather effects, its always blue skies
- sea level
- local solar time is approximated by local standard time
- ignores leap years
- declination angle is approximated by `d_0 * cos(2 * pi * (n+10)/366)`
  - `d_0 = 0.4093` declination at the northern winter solstice 

## Improvements
- use longitude to get solar time

[1] http://www.atmos.albany.edu/facstaff/brose/classes/ATM623_Spring2015/Notes/Lectures/Lecture11%20--%20Insolation.html
