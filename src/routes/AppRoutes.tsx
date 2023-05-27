import { Route, Routes as RoutesComponent } from 'react-router-dom';
import Test from './Test/Test';
import Test2 from './Test2/Test2';

const Routes = () => {
  return (
    <RoutesComponent>
      <Route path="/test" element={<Test />} />
      <Route path="/test2" element={<Test2 />} />
      <Route path="*" element={<div>nopage</div>} />
    </RoutesComponent>
  );
};

export default Routes;
