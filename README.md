# Finding-Busiest-Airports-Using-Degree-Distribution
## Introduction:

Degree distribution describes the connectivity of vertices in a graph through the
frequency distribution of degrees that are connected by edges across the network. This project
analyzes a graph that is composed of airports and routes, where the nodes are represented by
airports and edges are represented by routes. The degree of an airport is determined by the
amount of direct connection it has to other airports. Based on the routes provided by the
full_routes.csv (Full_Merge_of_All_Unique_Routes.csv) dataset, an airport is busier if its degree
is higher. Statistics of minimum, maximum, mean, medium, and the percentiles of airport
degrees are outputted, as well as a written file (Ranked Busiest Airports in the World by
Degrees.csv) of the airports listed from the most to least degrees. The analysis reveals statistics
about degrees at a distance of one neighbor, and a distance at two neighbors. At a distance of
two neighbors, the concept of second-degree connections can reveal further insight of how busy
an airport is in terms of how busy their neighbor airports are. Furthermore, the user is also
prompted to input two airport IDs to find the shortest amount of switches, or edge connections,
through the use of a Breadth-First-Search algorithm.

## Program:

The first part of the code’s output describes the statistics of airports for neighbors at distance 1
and 2. By analyzing the airport degrees at distance 1, we find that a vertex with higher degrees
is in a much smaller percentile than compared with lower degree vertices. For example, 95% of
the vertices have 100 or less degrees, while only 0.01% of vertices have 1750-2000 vertices. A
similar distribution is followed when you increase the distance to 2 neighbors. However,
increasing the distance creates a more evenly spread distribution, revealing a more connective
network.

The program then prompts the user to enter two airport IDs, one for a departure airport and
another for a destination airport. It then returns the shortest amount of switches needed to get
from the departure airport to the destination airport. The purpose of this function is to
understand the connectivity of busiest or non-busiest airports from other specific airports.

## Data Distribution Analysis

The distribution of degrees for the airports in Full_Merge_of_All_Unique_Routes.csv looks to be
like a power-law distribution, as the higher the degree an airport is, the more rare the node is
compared to other airports.

### Additional Notes

Although the project technically only requires full_routes.csv for connectivity analysis, I decided
to combine airports from full_airports.csv as well to match each ID to a corresponding airport.
Although not all airports in full_airports.csv are used by routes, I believe it further emphasizes
the distribution of how some airports are not as utilized as other airports.
