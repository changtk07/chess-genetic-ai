SOURCES := $(wildcard src/*.cpp)
OBJECTS := $(patsubst src/%.cpp,src/%.o,$(SOURCES))
DEPENDS := $(patsubst src/%.cpp,src/%.d,$(SOURCES))

WARNING := -Wall #-Wextra
STD := -std=c++17

.PHONY: all clean

all: chess

clean:
	$(RM) $(OBJECTS) $(DEPENDS) chess

chess: $(OBJECTS)
	$(CXX) $(WARNING) $(CXXFLAGS) $^ -o $@

-include $(DEPENDS)

%.o: %.cpp Makefile
	$(CXX) $(WARNING) $(STD) $(CXXFLAGS) -MMD -MP -c $< -o $@
