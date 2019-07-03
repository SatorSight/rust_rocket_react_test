import React from 'react';
import ReactDOM from 'react-dom';
import 'whatwg-fetch';

class IndexPage extends React.Component {
    constructor(props){
        super(props);

        this.state = {
            users: [],
            stagings: [],
        };
    }

    componentDidMount(){
        fetch('/')
            //todo set content-type on backend
            .then(r => r.json())
            .then(r => {
                this.setState({
                    users: r.users,
                    stagings: r.stagings
                });
            })
    }

    addUserStaging = () => {
        fetch('http://localhost:8000/add_user', {
            method: 'post'
        })
            .then(r => r.json())
            .then(r => console.log(r))
    };

    render() {
        return (
            <div className="container">
                <div className="users">
                    {this.state.users.map(user =>
                        <div key={user.id}>{user.name}</div>
                    )}
                </div>
                <div className="stagings">
                    {this.state.stagings.map(staging =>
                        <div key={staging.id}>{staging.name}</div>
                    )}
                </div>
                <button onClick={this.addUserStaging}>click me</button>
            </div>
        );
    }
}

ReactDOM.render(<IndexPage />, document.getElementById('root'));
