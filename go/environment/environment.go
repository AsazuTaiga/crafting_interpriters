package environment

type Environment struct {
	values map[string]interface{}
	Enclosing *Environment
}

func NewEnvironment(enclosing *Environment) *Environment {
	return &Environment{
		Enclosing: enclosing,
		values: make(map[string]interface{}),
	}
}

func (e *Environment) Define(name string, value interface{}) {
	e.values[name] = value
}

func (e *Environment) Get(name string) (interface{}, bool) {
	if value, ok := e.values[name]; ok {
		return value, ok
	}
	if e.Enclosing != nil {
		return e.Enclosing.Get(name)
	}
	return nil, false
}

func (e *Environment) Assign(name string, value interface{}) {
	_, ok := e.values[name]
	if ok {
		e.values[name] = value
		return 
	}
	if e.Enclosing != nil {
		e.Enclosing.Assign(name, value)
		return
	}
}
