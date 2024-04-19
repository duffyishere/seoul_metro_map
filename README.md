# 서울 지하철 최단 경로 구하기

[해당 동영상](https://www.youtube.com/watch?v=_y6sPa3TZz4)을 보고 이런 내용을 학교에서 배우는 서울대생이 부러워 나도 구현해봤다. 

## 조건
입력으로 출발역과 도착역이 필요하다. 출력으로는 역 사이의 거리 중 최솟값이 출력된다.\
만약, 환승역이 있을 경우 환승역을 강조하여 표시하고 환승시간만큼 총 소요시간에 추가한다.

## 구현 방법

1. 서울 지하철 노선도를 그래프 형식으로 변환하여 저징한다.
2. 다익스트라 알고리즘을 통해 출발지와 목적지 간의 최소 거리를 구한다.
3. 경로 중 환승 역이 있을 경우 강조 표시 후 최당 경로 출력.

<b>그래프 구성</b>
1. 인접 그래프: 역과 연간의 연결을(노선) 나타내는 그래프
2. 가중치 그래프: 역과 역간의 값(소요시간)을 나타내는 그래프

## 구현 중 문제점

<b>개별 지하철역에 대한 데이터는 구할 수 있지만, 어떻게 역 간의 노선을 간선으로 저장할 수 있을까?</b> 

1. 하나씩 수작업으로 데이터를 입력한다.
2. 국토교통부에서 도시철도 전체 노선 정보를 제공 중이다.

<b>환승역의 구분을 어떻게 해야할까?</b>

인접 그래프에서 인접한 역의 수가 3개 이상일 경우 환승역으로 구분할 수 있음.

<b>최소 소요 시간은 구할 수 있지만, 최적의 경로는 어떻게 구할 수 있을까?</b>

경로를 기록하는 변수를 추가한다.

## ISSUE

1. 2호선 지선의 길찾기는 지원되지 않습니다.
2. 환승 패널티가 없어 계속하여 환승하는 버그가 있습니다.

## TODO

- [ ] 경로 출력에서 환승역일 경우 강조 표시 추가
- [ ] 환승역 환승 소요시간 데이터 추가
- [ ] 역간의 소요시간 데이터 추가
- [ ] 목적지까지 최소 소요시간 정보 추가
