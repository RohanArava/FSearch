import LoadingSpin from "react-loading-spin";

export default function Loading(){ 
    return (
    <div className="d-flex w-100 h-100 align-items-center justify-content-center">
        <LoadingSpin
            duration="2s"
            width="5px"
            timingFunction="ease-in-out"
            primaryColor="#333"
            secondaryColor="#888"
            direction="alternate"
            size="200px"
            numberOfRotationsInAnimation={2}
        />
    </div>
);}