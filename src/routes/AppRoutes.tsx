import { Route, Routes as RoutesComponent } from 'react-router-dom';
import Test from './Test/Test';

const Routes = () => {
  return (
    <RoutesComponent>
      <Route path="/test" element={<Test />} />
      <Route path="*" element={<div>nopage</div>} />
    </RoutesComponent>
  );
};

export default Routes;
