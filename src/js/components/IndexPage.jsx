import React from 'react';
import ReactDOM from 'react-dom';
import 'whatwg-fetch';

class IndexPage extends React.Component {
    constructor(props){
        super(props);

        this.state = {
            users: [],
            stagings: [],
            users_stagings: [],

            new_user_name: '',
            new_staging_name: '',
        };
    }

    componentDidMount(){
        fetch('/')
            //todo set content-type on backend
            .then(r => r.json())
            .then(r => {
                this.setState({
                    users: r.users,
                    stagings: r.stagings,
                    users_stagings: r.users_stagings,
                });
            })
    }

    addUser = () =>
        this.fetchPost('add_user', { name: this.state.new_user_name })
            .then(r => r.json())
            .then(r => this.setState({ users: [r, ...this.state.users] }));

    addStaging = () =>
        this.fetchPost('add_staging', { name: this.state.new_staging_name })
            .then(r => r.json())
            .then(r => this.setState({ stagings: [r, ...this.state.stagings] }));

    assignStagingToUser = (staging_id, user_id) =>
        this.fetchPost('assign_staging_to_user', {
            user_id: user_id,
            staging_id: staging_id
        })
            .then(r => r.json())
            .then(r => console.log(r));

    fetchPost = (path, data) => {
        return fetch(`http://localhost:8000/${path}`, {
            method: 'POST',
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(data)
        })
    };

    delete = (type, id) => {
        const smth = type === 'user' ? this.state.users : this.state.stagings;
        const key = `${type}_id`;
        const key2 = `${type}s`;
        const data = { [`${type}_id`]: id };
        const state = { [`${type}s`]: smth };
        // data[key] = id;
        // const data = { id: id };
        return fetch(`http://localhost:8000/${type}`, {
            method: 'DELETE',
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(data)
        })
            .then(r => r.json())
            .then(r => console.log(r))
            .then(() => this.setState({ [key2]: smth.filter(s => s.id !== id) }))
    };

    onStaging = (user, staging) =>
        this.state.users_stagings.find(us => us.user_id === user.id && us.staging_id === staging.id);

    options = staging =>
        this.state.users.map(u =>
            <option key={u.id} selected={this.onStaging(u, staging)} value={u.id}>{u.name}</option>);

    changeNewUserName = e => this.setState({ new_user_name: e.target.value });
    changeNewStagingName = e => this.setState({ new_staging_name: e.target.value });

    toggleBusy = staging => {
        const data = { staging_id: staging.id };
        return fetch('http://localhost:8000/staging', {
            method: 'PATCH',
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(data)
        })
            .then(r => r.json())
            .then(r => console.log(r))
            .then(() => {
                // let st = this.state.stagings.find(s => s.id === staging.id);
                // st.busy = false;
                this.setState({ stagings: this.state.stagings.map(s => {
                    if(s.id === staging.id)
                        s.busy = !s.busy;
                    return s;
                })})
            })

    };

    render() {
        return (
            <div className="container">
                {this.state.stagings.map(s =>
                    <div key={s.id} className="row">
                        <div className="staging_name">{s.name}</div>
                        <div className="staging_user">
                            <select onChange={e => this.assignStagingToUser(s.id, +e.target.value)}>
                                {this.options(s)}
                            </select>
                        </div>
                        <div className="toggleBusy">
                            <button onClick={() => this.toggleBusy(s)} style={s.busy ? {color: 'red'} : {color: 'green'}}>
                                {s.busy ? 'Busy' : 'Free'}
                            </button>
                        </div>
                    </div>
                )}
                <br/>
                <br/>
                <div>USERS</div>
                <div className="users">
                    {this.state.users.map(user =>
                        <div key={user.id}>
                            <span>{user.name}</span>
                            <span onClick={() => this.delete('user', user.id)}>x</span>
                        </div>
                    )}
                </div>
                <br/>
                <br/>
                <div>STAGINGS</div>
                <div className="stagings">
                    {this.state.stagings.map(staging =>
                        <div key={staging.id}>
                            <span>{staging.name}</span>
                            <span onClick={() => this.delete('staging', staging.id)}>x</span>
                        </div>
                    )}
                </div>
                <input type="text" value={this.state.new_user_name} onChange={this.changeNewUserName}/>
                <button onClick={this.addUser}>add user</button>
                <input type="text" value={this.state.new_staging_name} onChange={this.changeNewStagingName}/>
                <button onClick={this.addStaging}>add staging</button>
                <button onClick={() => this.assignStagingToUser(1,1)}>add first staging to first</button>
            </div>
        );
    }
}

ReactDOM.render(<IndexPage />, document.getElementById('root'));
